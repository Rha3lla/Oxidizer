# Rust Static Analyzer

**A rules-driven static analysis tool for Rust — designed for high-assurance systems, embedded development, and CI/CD integration.**  
This project aims to fill the gap between compiler-level checks and the rigorous demands of safety, security, and embedded coding standards.

---

## 📌 Why This Tool Exists

Rust today benefits from a strong type system and ownership model, but:
- The built-in tools (`rustc`, Clippy) focus on correctness, style, or ergonomic warnings — *not high-assurance policy enforcement*.
- There is no first-class Rust analyzer tailored for **embedded constraints**, **safety/mission-critical environments**, or **security policy alignment**.
- Teams building on bare-metal and cross-compiling for diverse targets need a **ruleset engine** that works equally well on Linux and Windows, integrates seamlessly into pipelines, and adapts to project-specific policies.

This analyzer is being built to solve that need.

---
## Planned Features
- [ ] MISRA-C:2025 Addendum 6 rule coverage
- [ ] SARIF 2.1.0 compliant output
- [ ] Severity classification (Critical/High/Medium/Low)
- [ ] Cross-platform support (Windows/Linux)
- [ ] Extensible rule architecture
- [ ] Compliance reporting for certification packages

---
## Target Users
- Department of Defense programs requiring software certification
- Military services undergoing NSA high-assurance evaluation
- Defense contractors building safety/security-critical Rust systems

---

## 🛠️ Usage

> ⚠️ This tool is under active development, infrastructure phase 



GOALS
1. Determine rulesets and requirements that need to go into the tool.
2. 2. Find any/all CWE/CVEs against Rust to incorporate into the tool.
3. Need to be platform agnostic so it can be run on both a Linux and a Windows environment.
4. Need to be chip agnostic so it can be run against bare-metal or OS-based implementations.
5. Ideally this would be something developers could implement as part of their CI/CD pipeline to run against code developed as part of regular testing
6. There should be rules that comply with MISRA coding standards as they apply to Rust
7. There will probably be more requirements that are discovered along the way. 




Acknowledgements
Inspired by limitations in existing tooling like rustc and Clippy
Leveraging the RustSec Advisory Database
Aligned to emerging Rust safety-critical practices


## License
Proprietary — see COPYRIGHT
## Contact
Melissa Wilson, medicmommy83@gmail.com
