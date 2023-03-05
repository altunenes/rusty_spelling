# rusty_spelling
A basic spelling game in Rust programing language

Description:
- The user enters the words they want to practice.

- The user tries to write the word they hear in a proper way.
- Print /exit to exit the program and it prints the score and statistics of the game.
- You can play with "word_list.txt" file that contains a list of words. You can add your own words to the file.
- When you run a game it will automatically create a result.json file that contains the statistics of the game so you can track your progress.

### Usage/Installation:
1-   You need to install rust on your computer. You can install it from [here](https://www.rust-lang.org/tools/install).

2- Click on the green "Code" button (top right of this page) and select "Download ZIP".

3- Extract the downloaded ZIP file to a folder on your computer (e.g., "rusty_spelling").
   Open the folder "rusty_spelling".

4- Open the terminal (also known as command prompt or cmd) on your computer. You can simply open it in the folder "rusty_spelling" by right-clicking on the folder and selecting "Open in Terminal".

5- Write the following command in the terminal to compile the code and create an executable file.
  
```bash
cargo build --release
```

- This will compile code and create an executable in the "target/release/rusty_spelling.exe" directory.


- You can simply the run executable file by double-clicking on it. 


- Or run this command in the terminal after you build the executable file.
  
```bash
cargo run
```

This will compile and run the program.


  ![8x8](./static/rusty_spelling.png)



### Motivation:
- My little sister studying English and I wanted to make a simple spelling game for her to practice her English :)

#### Future, in progress:

- Working on a web version via wasm.
