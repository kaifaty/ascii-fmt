#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const colors = {
  reset: '\x1b[0m',
  green: '\x1b[32m',
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function error(message) {
  log(message, 'red');
  process.exit(1);
}

function exec(command, options = {}) {
  try {
    return execSync(command, {
      stdio: options.silent ? 'pipe' : 'inherit',
      ...options,
    }).toString();
  } catch (err) {
    if (options.silent) return null;
    error(`Command failed: ${command}\n${err.message}`);
  }
}

function checkRust() {
  log('Checking Rust installation...', 'cyan');

  const version = exec('rustc --version', { silent: true });
  if (version) {
    log(`Rust installed: ${version.trim()}`, 'green');
    return true;
  }

  log('Rust not found', 'yellow');
  return false;
}

function installRust() {
  log('Installing Rust...', 'cyan');

  const platform = os.platform();
  let installCommand = 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y';

  if (platform === 'win32') {
    log('On Windows, please install Rust manually: https://rustup.rs/', 'yellow');
    log('After installation, run this script again.', 'yellow');
    process.exit(1);
  }

  log('Running rustup installer (this may take a few minutes)...', 'cyan');

  try {
    execSync(installCommand, { stdio: 'inherit' });

    const cargoHome = process.env.CARGO_HOME || path.join(os.homedir(), '.cargo');
    const envScript = path.join(cargoHome, 'env');

    if (fs.existsSync(envScript)) {
      const envContent = fs.readFileSync(envScript, 'utf8');
      envContent.split('\n').forEach(line => {
        const match = line.match(/^export (\w+)="(.+)"$/);
        if (match) {
          process.env[match[1]] = match[2].replace(/\$HOME/g, os.homedir());
        }
      });
    }

    log('Rust installed successfully!', 'green');
  } catch (err) {
    error('Failed to install Rust. Please install manually from https://rustup.rs/');
  }
}

function build() {
  log('Building ascii-fmt...', 'cyan');

  const projectRoot = path.dirname(__dirname);

  try {
    exec('cargo build --release', { cwd: projectRoot });

    let binaryPath;
    const platform = os.platform();
    const arch = os.arch();

    if (platform === 'win32') {
      binaryPath = path.join(projectRoot, 'target', 'release', 'ascii-fmt.exe');
    } else {
      binaryPath = path.join(projectRoot, 'target', 'release', 'ascii-fmt');
    }

    if (!fs.existsSync(binaryPath)) {
      error(`Binary not found at expected location: ${binaryPath}`);
    }

    log(`Build successful! Binary: ${binaryPath}`, 'green');

    const binDir = path.join(projectRoot, 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }

    const targetPath = platform === 'win32'
      ? path.join(binDir, 'ascii-fmt.exe')
      : path.join(binDir, 'ascii-fmt');

    fs.copyFileSync(binaryPath, targetPath);

    if (platform !== 'win32') {
      fs.chmodSync(targetPath, 0o755);
    }

    log(`Binary copied to: ${targetPath}`, 'green');

    log('', 'reset');
    log('To use ascii-fmt from anywhere, add this directory to your PATH:', 'yellow');
    log(`  ${binDir}`, 'cyan');
    log('', 'reset');
    log('Or install globally with:', 'yellow');
    log('  cargo install --path .', 'cyan');
    log('', 'reset');

    return targetPath;
  } catch (err) {
    error(`Build failed: ${err.message}`);
  }
}

function main() {
  log('========================================', 'cyan');
  log('  ascii-fmt Build Script', 'cyan');
  log('========================================', 'cyan');
  log('', 'reset');

  if (!checkRust()) {
    installRust();
  }

  const binaryPath = build();

  log('', 'reset');
  log('✓ Build completed successfully!', 'green');
  log('', 'reset');
}

main();
