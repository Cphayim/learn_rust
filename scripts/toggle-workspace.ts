#!/usr/bin/env tsx
import fs from 'node:fs'
import path from 'node:path'

const key = process.argv[2]

const map = {
  'the-book': 'the-rust-programming-language',
  geektime: 'geektime-rust',
}

if (!map[key]) {
  console.error(`Unknown workspace: ${key}, must be one of [${Object.keys(map).join(', ')}]`)
  process.exit(1)
}

const rootDir = path.resolve(__dirname, '..')
const targetDir = path.resolve(rootDir, map[key])
const learnRustWorkspace = path.resolve(rootDir, 'learn-rust.code-workspace')

if (!fs.existsSync(targetDir)) {
  console.error('Workspace not found:', targetDir)
  process.exit(2)
}

const workspaces = fs
  .readdirSync(targetDir)
  .filter((item) => fs.statSync(path.join(targetDir, item)).isDirectory())
  .sort((a, b) => parseFloat(a.split('-')[0]) - parseFloat(b.split('-')[0]))

type Folder = {
  name: string
  path: string
}

const rootFolder: Folder = {
  name: '✨ learn-rust',
  path: '.',
}

const folders = workspaces.map<Folder>((item) => ({ name: item, path: path.join(map[key], item) }))
folders.unshift(rootFolder)

const originalWorkspace = fs.existsSync(learnRustWorkspace)
  ? JSON.parse(fs.readFileSync(learnRustWorkspace, 'utf8'))
  : {}

fs.writeFileSync(learnRustWorkspace, JSON.stringify({ ...originalWorkspace, folders }, null, 2))
