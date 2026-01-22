#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function fatal(message) {
  process.stderr.write(`${message}\n`);
  process.exit(1);
}

function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function writeFileIfMissing(filePath, content) {
  if (fs.existsSync(filePath)) return false;
  fs.writeFileSync(filePath, content, 'utf8');
  return true;
}

function main() {
  const projectRoot = process.cwd();
  const opencodeDir = path.join(projectRoot, '.opencode');
  const pluginsDir = path.join(opencodeDir, 'plugins');
  const pluginPath = path.join(pluginsDir, 'ascii-fmt.js');

  ensureDir(pluginsDir);

  const templatePath = path.join(__dirname, 'opencode-plugin.js');
  const template = fs.readFileSync(templatePath, 'utf8');
  const pluginCreated = writeFileIfMissing(pluginPath, template);

  if (pluginCreated) {
    process.stdout.write(`Created OpenCode plugin: ${path.relative(projectRoot, pluginPath)}\n`);
  } else {
    process.stdout.write(`OpenCode plugin already exists: ${path.relative(projectRoot, pluginPath)}\n`);
  }

  process.stdout.write('\nNext:\n');
  process.stdout.write('- Ensure ascii-fmt is installed and on PATH\n');
  process.stdout.write('- In Markdown, mark diagram blocks as ```ascii / ```diagram / ```ascii-diagram\n');
}

main();
