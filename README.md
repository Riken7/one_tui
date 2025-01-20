# one-tui

A terminal client for OneDrive, built in Rust.
![one_tui screenshot]![Screenshot from 2025-01-20 10-57-57 1920x1080](https://github.com/user-attachments/assets/fe6db6f5-eff3-45ef-95cf-bd6f9c1454ec)

> [!WARNING]  
> `one_tui` is in early development! Expect horrible bugs and unexpected behavior.
> This project may not support all types of OneDrive accounts.

> [!IMPORTANT]  
> **DISCLAIMER:** `one_tui` is developed and maintained in free time voluntarily.

## Prerequisites

To use `one_tui`, you need to have the following installed:

1. **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
2. **Zenity/Kdialog**: for file picker support

#### For Ubuntu/Debian-based systems:
    sudo apt update
    sudo apt install zenity
#### For Arch Linux:
    sudo pacman -S zenity

3. **Hack Mono Nerd Font**: [Download here](https://www.nerdfonts.com/)

## Development

To build and run one_tui, follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/Riken7/one_tui.git
    cd one_tui
    ```
2. Run 
    ```bash
    cargo run
    ```
3. To build project
    ```bash
    cargo build --release
    ```

