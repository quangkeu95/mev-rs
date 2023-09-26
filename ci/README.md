# Rust CI configuration

## Github Actions
To add Rust Github Actions CI configuration to your project:
1. Add Git remote tracked repository for CI configuration:
```bash
git remote add ci https://github.com/quangkeu95/rust-ci-conf
```

2. Fetch from remote CI configuration repository:
```bash
git fetch ci
```

3. Merge your desired CI configuration repository branch into your project:
```bash
git merge --allow-unrelated ci/main
```

or 
```bash
git merge --allow-unrelated ci/minimal
```

All-in-one command:
```bash
git remote add ci https://github.com/quangkeu95/rust-ci-conf
git fetch ci
git merge --allow-unrelated ci/main
```

## Acknowledges:
[JonHoo Rust CI Config](https://github.com/jonhoo/rust-ci-conf)
