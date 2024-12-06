# 🌐 Crust - CRDTs in Rust

Welcome to **Crust**, the basic implementation of **CRDTs (Conflict-free Replicated Data Types)** written in **Rust**! This project provides a foundation for CRDTs by implementing several data types. including:

Each data type will include implementations of **CmRDT**, **CvRDT**, and **Delta**, giving you a comprehensive understanding of how these concepts work and interact.

- **CmRDT**: Commutative Replicated Data Types
- **CvRDT**: Convergent Replicated Data Types
- **Delta**: Delta-based CRDTs

## ⚠️ Disclaimer

**Crust** is currently in the early stages of development. The project only includes the basic structure for various CRDT data types, with no full implementation of the algorithms or features yet. This project is a work in progress, and the CRDT types (such as counters, graphs, registers, sets, and maps) are not yet functional.

## 📋 Table of Contents

- [🌐 Crust - CRDTs in Rust](#-crust---crdts-in-rust)
  - [⚠️ Disclaimer](#️-disclaimer)
  - [📋 Table of Contents](#-table-of-contents)
  - [📦 Features](#-features)
  - [🛠️ To-Do](#️-to-do)
    - [🏗️ **Counter**](#️-counter)
    - [🧑‍🤝‍🧑 **Graph**](#-graph)
    - [🗺️ **Map**](#️-map)
    - [🖊️ **Register**](#️-register)
    - [🛑 **Set**](#-set)
    - [🧪 **Tests**](#-tests)
  - [🤝 Contributing](#-contributing)
  - [📄 License](#-license)

## 📦 Features

- ⚙️ **Multiple CRDT types**: Including sets, counters, and maps.
- 🧩 **Modular design**: CmRDT, CvRDT, and Delta types for each data structure.
- 🦀 **Rust-based**: Written entirely in Rust for performance and safety.

## 🛠️ To-Do

<details> <summary>Tap to expand</summary>

### 🏗️ **Counter**

- [ ] **GCounter**: Implement a grow-only counter.
- [ ] **PNCounter**: Implement a positive-negative counter.

### 🧑‍🤝‍🧑 **Graph**

- [ ] **AWGraph**: Implement an add-wins graph.
- [ ] **GGraph**: Implement a grow-only graph.
- [ ] **ORGraph**: Implement an observed-remove graph.
- [ ] **TPGraph**: Implement a 2-phase graph.

### 🗺️ **Map**

- [ ] Implement a **Map** CRDT.

### 🖊️ **Register**

- [ ] **LWWRegister**: Implement a last-write-wins register.
- [ ] **MVRegister**: Implement a multi-value register.

### 🛑 **Set**

- [ ] **AWSet**: Implement an add-wins set.
- [ ] **GSet**: Implement a grow-only set.
- [ ] **ORSet**: Implement an observed-remove set.
- [ ] **RWSet**: Implement a read-write set.
- [ ] **TPSet**: Implement a two-phase set.

### 🧪 **Tests**

- [ ] Write unit tests for **GCounter**.
- [ ] Write unit tests for **PNCounter**.
- [ ] Write unit tests for **AWGraph**.
- [ ] Write unit tests for **GGraph**.
- [ ] Write unit tests for **ORGraph**.
- [ ] Write unit tests for **TPGraph**.
- [ ] Write unit tests for **Map**.
- [ ] Write unit tests for **LWWRegister**.
- [ ] Write unit tests for **MVRegister**.
- [ ] Write unit tests for **AWSet**.
- [ ] Write unit tests for **GSet**.
- [ ] Write unit tests for **ORSet**.
- [ ] Write unit tests for **RWSet**.
- [ ] Write unit tests for **TPSet**.

</details>

## 🤝 Contributing

We welcome contributions! If you’d like to contribute, please fork this repository, create a branch, and submit a pull request.

Make sure to follow these steps:

1. Fork this repo
2. Clone your fork: `git clone https://github.com/zz0-0/crust.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Commit your changes: `git commit -m 'Add new feature'`
5. Push to your fork: `git push origin feature/your-feature`
6. Create a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
