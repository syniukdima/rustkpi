document.addEventListener("DOMContentLoaded", () => {
    const taskListElement = document.getElementById("task-list");
    const addTaskForm = document.getElementById("add-task-form");
    const taskDescInput = document.getElementById("task-desc");

    // Load task list from the server
    async function loadTasks() {
        const response = await fetch("/tasks");
        const data = await response.json();
        renderTasks(data.tasks);
    }

    // Render the task list
    function renderTasks(tasks) {
        taskListElement.innerHTML = ""; // Clear the list before rendering
        tasks.forEach(task => {
            const taskElement = document.createElement("li");
            taskElement.className = `list-group-item d-flex justify-content-between align-items-center ${
                task.completed ? "list-group-item-success" : ""
            }`;

            // Task description
            const taskText = document.createElement("span");
            taskText.textContent = task.description;

            // Control buttons
            const buttonsContainer = document.createElement("div");

            // Mark as completed/undo button
            const toggleButton = document.createElement("button");
            toggleButton.className = "btn btn-sm btn-outline-success me-2";
            toggleButton.textContent = task.completed ? "Undo" : "Complete";
            toggleButton.onclick = async () => {
                task.completed = !task.completed;
                await updateTask(task);
                await loadTasks();
            };

            // Delete button
            const deleteButton = document.createElement("button");
            deleteButton.className = "btn btn-sm btn-outline-danger me-2";
            deleteButton.textContent = "Delete";
            deleteButton.onclick = async () => {
                await deleteTask(task.id);
                await loadTasks();
            };

            // Edit button
            const editButton = document.createElement("button");
            editButton.className = "btn btn-sm btn-outline-primary me-2";
            editButton.textContent = "Edit";
            editButton.onclick = () => {
                const newDescription = prompt("Edit task description:", task.description);
                if (newDescription && newDescription.trim() !== task.description) {
                    task.description = newDescription.trim();
                    updateTask(task);
                    loadTasks();
                }
            };

            buttonsContainer.appendChild(toggleButton);
            buttonsContainer.appendChild(deleteButton);
            buttonsContainer.appendChild(editButton); // Add edit button

            taskElement.appendChild(taskText);
            taskElement.appendChild(buttonsContainer);
            taskListElement.appendChild(taskElement);
        });
    }

    // Add a new task
    addTaskForm.addEventListener("submit", async (event) => {
        event.preventDefault();
        const description = taskDescInput.value.trim();
        if (description) {
            const newTask = { id: Date.now(), description, completed: false };
            await addTask(newTask);
            taskDescInput.value = ""; // Clear the input field
            await loadTasks();
        }
    });

    // API calls

    async function addTask(task) {
        await fetch("/add", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(task),
        });
    }

    async function deleteTask(id) {
        await fetch(`/delete/${id}`, {
            method: "POST",
        });
    }

    async function updateTask(task) {
        await fetch(`/update/${task.id}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(task),
        });
    }

    // Initialization
    loadTasks();
});
