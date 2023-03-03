const inputWord = document.getElementById("input-word");
const submitButton = document.getElementById("submit-button");
const result = document.getElementById("result");

submitButton.addEventListener("click", () => {
    const word = inputWord.value.trim();
    if (word === "") {
        result.textContent = "Please enter a word.";
        return;
    }

    fetch(`/api/check-spelling?word=${word}`)
        .then(response => response.json())
        .then(data => {
            if (data.correct) {
                result.textContent = `"${word}" is spelled correctly!`;
            } else {
                result.textContent = `"${word}" is not spelled correctly. The correct spelling is "${data.correctSpelling}".`;
            }
        })
        .catch(() => {
            result.textContent = "An error occurred. Please try again.";
        });
});