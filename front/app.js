const taskInput = document.getElementById("task-input");
const addTaskButton = document.getElementById("add-task-button");
const tasksList = document.getElementById("tasks-list");

addTaskButton.addEventListener("click", async () => {
    const task = taskInput.value.trim();
    if (task) {
        await fetch("http://127.0.0.1:8080/add_task", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ task }),
        });
        taskInput.value = "";
        await loadTasks();
    }
});

tasksList.addEventListener("click", async (event) => {
    if (event.target.classList.contains("remove-task-button")) {
        const index = parseInt(event.target.dataset.index, 10);
        await fetch(`http://127.0.0.1:8080/remove_task/${index}`, {
            method: "DELETE",
        });
        await loadTasks();
    }
});

async function loadTasks() {
    const response = await fetch("http://127.0.0.1:8080/show_tasks");
    const tasks = await response.json();
    tasksList.innerHTML = tasks
        .map(
            (task, index) => `
            <li class="task-item">
                ${task}
                <button class="remove-task-button" data-index="${index + 1}">Remove</button>
            </li>
        `
        )
        .join("");
}

loadTasks();
