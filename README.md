# rusty_spelling

[![Rust](https://github.com/altunenes/rusty_spelling/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/rusty_spelling/actions/workflows/rust.yml)

![rustyse](https://user-images.githubusercontent.com/54986652/227798426-65315fd5-ad30-4f41-b384-5333dcc84d49.png)

:crab: :crab: spelling game in Rust programing language :crab: :crab:

Description:
- The user enters the words they want to practice.

- The user tries to write the word they hear in a proper way.
- Print /exit to exit the program and it prints the score and statistics of the game.
- You can play with "word_list.txt" file that contains a list of words. You can add your own words to the file.
- When you run a game it will automatically create a result.json file that contains the statistics of the game so you can track your progress. You can also play with the previous mistakes you made with this JSON file.
- You can set the pronunciation speed when you run the program. The default is 1.0. You can adjust it from 0.5 to 10. Here is an example:

```bash
cargo run -- -s 0.6
```

### Usage/Installation:
1-   You need to install rust on your computer. You can install it from [here](https://www.rust-lang.org/tools/install).

2- Click on the green "Code" button (top right of this page) and select "Download ZIP".

3- Extract the downloaded ZIP file to a folder on your computer (e.g., "rusty_spelling").
   Open the folder "rusty_spelling".

4- Open the terminal (also known as command prompt or cmd) on your computer. You can simply open it in the folder "rusty_spelling" by right-clicking on the folder and selecting "Open in Terminal".


5- Just write cargo run in the terminal to run the program like this:

```bash
cargo run
```

Thats it! You can start practicing your spelling :)

  
6- Optionally, you can add the "--release" flag to the command to compile the code in release mode. This will make the executable file smaller and faster.

```bash
cargo build --release
```

- This will compile code and create an executable in the "target/release/rusty_spelling.exe" directory.


- You can simply the run executable file by double-clicking on it. 


Current look of the game:

  ![8x8](./static/rusty_spelling.png)



### Motivation:
- My little sister studying English and I wanted to make a simple spelling game for her to practice her English :)

#### Future, in progress:

- Working on a web version via wasm. :crab:
- :no_entry_sign: react  

### Contributing / Questions / Issues:
- Feel free to open an issue or a pull request if you want to contribute to the project.
