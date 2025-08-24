#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

// Function to extract version from Cargo.toml as fallback
function getVersionFromCargoToml() {
  try {
    const cargoPath = path.join(__dirname, '..', '..', 'Cargo.toml');
    const cargoContent = fs.readFileSync(cargoPath, 'utf8');
    const versionMatch = cargoContent.match(/^version\s*=\s*"([^"]+)"/m);
    return versionMatch ? versionMatch[1] : null;
  } catch (error) {
    console.warn('⚠️  Could not read Cargo.toml for version fallback:', error.message);
    return null;
  }
}

// Enhanced version detection with multiple fallbacks
let version = null;

// 1. Try GITHUB_REF environment variable (for tagged releases)
if (process.env.GITHUB_REF && process.env.GITHUB_REF.startsWith('refs/tags/v')) {
  version = process.env.GITHUB_REF.replace('refs/tags/v', '');
  console.log(`📍 Version from GITHUB_REF: ${version}`);
}

// 2. Try command line argument
if (!version && process.argv[2]) {
  version = process.argv[2];
  console.log(`📍 Version from command line: ${version}`);
}

// 3. Try Cargo.toml as fallback
if (!version) {
  version = getVersionFromCargoToml();
  if (version) {
    console.log(`📍 Version from Cargo.toml (fallback): ${version}`);
  }
}

// 4. Final validation
if (!version || !/^\d+\.\d+\.\d+/.test(version)) {
  console.error('❌ Error: No valid version could be determined');
  console.error('');
  console.error('Tried the following sources:');
  console.error('  1. GITHUB_REF environment variable (refs/tags/v*)');
  console.error('  2. Command line argument (node prepare-packages.js 1.0.0)');
  console.error('  3. Cargo.toml version field');
  console.error('');
  console.error('Please provide version via one of these methods.');
  process.exit(1);
}

console.log(`🚀 Preparing packages for version ${version}`);

// Define platform structures
const platforms = [
  'darwin-x64',
  'darwin-arm64', 
  'linux-x64',
  'linux-x64-musl',
  'win32-x64'
];

// Prepare platform packages
platforms.forEach(platform => {
  const sourceDir = path.join(__dirname, '..', 'platforms', platform);
  const targetDir = path.join(__dirname, '..', '..', 'npm-publish', platform);
  
  // Create directory
  fs.mkdirSync(targetDir, { recursive: true });
  
  // Read template package.json
  const templatePath = path.join(sourceDir, 'package.json');
  const packageJson = JSON.parse(fs.readFileSync(templatePath, 'utf8'));
  
  // Update version
  packageJson.version = version;
  
  // Write to target directory
  fs.writeFileSync(
    path.join(targetDir, 'package.json'),
    JSON.stringify(packageJson, null, 2) + '\n'
  );
  
  console.log(`✓ Prepared @ding113/ccline-packycc-${platform} v${version}`);
});

// Prepare main package
const mainSource = path.join(__dirname, '..', 'main');
const mainTarget = path.join(__dirname, '..', '..', 'npm-publish', 'main');

// Copy main package files
fs.cpSync(mainSource, mainTarget, { recursive: true });

// Update main package.json
const mainPackageJsonPath = path.join(mainTarget, 'package.json');
const mainPackageJson = JSON.parse(fs.readFileSync(mainPackageJsonPath, 'utf8'));

mainPackageJson.version = version;

// Update optionalDependencies versions
if (mainPackageJson.optionalDependencies) {
  Object.keys(mainPackageJson.optionalDependencies).forEach(dep => {
    if (dep.startsWith('@ding113/ccline-packycc-')) {
      mainPackageJson.optionalDependencies[dep] = version;
    }
  });
}

fs.writeFileSync(
  mainPackageJsonPath,
  JSON.stringify(mainPackageJson, null, 2) + '\n'
);

console.log(`✓ Prepared @ding113/ccline-packycc v${version}`);
console.log(`\n🎉 All packages prepared for version ${version}`);

// Validation step
console.log('\n🔍 Validating prepared packages...');
let validationPassed = true;

// Validate platform packages
platforms.forEach(platform => {
  const targetDir = path.join(__dirname, '..', '..', 'npm-publish', platform);
  const packageJsonPath = path.join(targetDir, 'package.json');
  
  if (!fs.existsSync(packageJsonPath)) {
    console.error(`❌ Missing package.json for ${platform}`);
    validationPassed = false;
    return;
  }
  
  try {
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    if (packageJson.version !== version) {
      console.error(`❌ Version mismatch for ${platform}: expected ${version}, got ${packageJson.version}`);
      validationPassed = false;
    } else {
      console.log(`✅ ${platform}: package.json valid`);
    }
  } catch (error) {
    console.error(`❌ Invalid package.json for ${platform}: ${error.message}`);
    validationPassed = false;
  }
});

// Validate main package (reuse existing mainPackageJsonPath)
if (!fs.existsSync(mainPackageJsonPath)) {
  console.error('❌ Missing main package.json');
  validationPassed = false;
} else {
  try {
    const validationPackageJson = JSON.parse(fs.readFileSync(mainPackageJsonPath, 'utf8'));
    if (validationPackageJson.version !== version) {
      console.error(`❌ Version mismatch for main package: expected ${version}, got ${validationPackageJson.version}`);
      validationPassed = false;
    } else {
      console.log('✅ main: package.json valid');
    }
  } catch (error) {
    console.error(`❌ Invalid main package.json: ${error.message}`);
    validationPassed = false;
  }
}

if (!validationPassed) {
  console.error('\n❌ Package validation failed!');
  process.exit(1);
}

console.log('\n✅ All packages validated successfully!');
console.log('\nNext steps:');
console.log('1. Copy binaries to platform directories');
console.log('2. Publish platform packages first');
console.log('3. Publish main package last');