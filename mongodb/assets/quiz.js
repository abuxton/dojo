document.querySelectorAll("[data-quiz]").forEach((quiz) => {
  const button = quiz.querySelector("button");
  const feedback = quiz.querySelector(".feedback");

  button.addEventListener("click", () => {
    const selected = quiz.querySelector("input:checked");

    if (!selected) {
      feedback.textContent = "Choose an answer first.";
      feedback.className = "feedback incorrect";
      return;
    }

    if (selected.value === "correct") {
      feedback.textContent = quiz.dataset.success;
      feedback.className = "feedback correct";
      return;
    }

    feedback.textContent = quiz.dataset.retry;
    feedback.className = "feedback incorrect";
  });
});
