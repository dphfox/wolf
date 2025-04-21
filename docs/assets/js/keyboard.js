document.addEventListener("keydown", (event) => {
	if (event.isComposing || event.keyCode === 229) {
		return
	}
	if (event.key === "ArrowLeft") {
		let keyboardBack = document.querySelector("#keyboard-back")
		if (keyboardBack != null) {
			keyboardBack.click()
		}
	} else if (event.key === "ArrowRight") {
		let keyboardNext = document.querySelector("#keyboard-next")
		if (keyboardNext != null) {
			keyboardNext.click()
		}
	}
});