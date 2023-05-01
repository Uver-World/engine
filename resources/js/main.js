Neutralino.init();

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

let square = { x: 50, y: 50, dragging: false, dragStartX: 0, dragStartY: 0, isDragable: true, fillStyle: "#FF0000" };
let squares = [];

let drawSquare = (x, y, fillStyle) => {
    context.fillStyle = fillStyle;
    context.fillRect(x, y, 100, 100);
}

canvas.addEventListener("mousedown", (event) => {
    let mouseX = event.clientX - canvas.offsetLeft;
    let mouseY = event.clientY - canvas.offsetTop;
    for (let i = squares.length - 1; i >= 0; i--) {
        if (mouseX >= squares[i].x && mouseX <= squares[i].x + 100 && mouseY >= squares[i].y && mouseY <= squares[i].y + 100) {
            if (!squares[i].isDragable) {
                openSquareContextMenu(squares[i]);
                break;
            }
            squares[i].dragging = true;
            squares[i].dragStartX = mouseX - squares[i].x;
            squares[i].dragStartY = mouseY - squares[i].y;
            break;
        }
    }
});

canvas.addEventListener("mousemove", (event) => {
    for (let i = squares.length - 1; i >= 0; i--) {
        if (squares[i].dragging) {
            let mouseX = event.clientX - canvas.offsetLeft;
            let mouseY = event.clientY - canvas.offsetTop;
            squares[i].x = mouseX - squares[i].dragStartX;
            squares[i].y = mouseY - squares[i].dragStartY;
            context.clearRect(0, 0, canvas.width, canvas.height);
            for (let j = 0; j < squares.length; j++) {
                drawSquare(squares[j].x, squares[j].y, squares[j].fillStyle);
            }
            break;
        }
    }
});

canvas.addEventListener("mouseup", (event) => {
    for (let i = squares.length - 1; i >= 0; i--) {
        if (squares[i].dragging) {
            squares[i].dragging = false;
            let newSquare = Object.assign({}, square);;
            squares.push(newSquare);
            drawSquare(newSquare.x, newSquare.y, square.fillStyle);
            squares[i].isDragable = false;
            break;
        }
    }
});

let openSquareContextMenu = (square) => {
    console.log(square);
    const popup = window.open("", "popup", "width=300,height=300");

    const title = document.createElement("h1");
    title.innerText = "Options du carré";

    const form = document.createElement("form");
    form.style.display = "flex";
    form.style.flexDirection = "column";

    const xLabel = document.createElement("label");
    xLabel.innerText = "Position X : ";
    const xInput = document.createElement("input");
    xInput.type = "number";
    xInput.value = square.x;
    xInput.addEventListener("input", () => {
        square.x = Number(xInput.value);
    });
    xLabel.appendChild(xInput);
    form.appendChild(xLabel);

    const yLabel = document.createElement("label");
    yLabel.innerText = "Position Y : ";
    const yInput = document.createElement("input");
    yInput.type = "number";
    yInput.value = square.y;
    yInput.addEventListener("input", () => {
        square.y = Number(yInput.value);
    });
    yLabel.appendChild(yInput);
    form.appendChild(yLabel);

    const isDragableLabel = document.createElement("label");
    isDragableLabel.innerText = "Est déplaçable : ";
    const isDragableInput = document.createElement("input");
    isDragableInput.type = "checkbox";
    isDragableInput.checked = square.isDragable;
    isDragableInput.addEventListener("change", () => {
        square.isDragable = isDragableInput.checked;
    });
    isDragableLabel.appendChild(isDragableInput);
    form.appendChild(isDragableLabel);

    const colorLabel = document.createElement("label");
    colorLabel.innerText = "Couleur : ";
    const colorInput = document.createElement("input");
    colorInput.type = "color";
    colorInput.value = square.fillStyle;
    colorInput.addEventListener("input", () => {
        square.fillStyle = colorInput.value;
    });
    colorLabel.appendChild(colorInput);
    form.appendChild(colorLabel);

    const submitButton = document.createElement("button");
    submitButton.innerText = "Valider";
    submitButton.type = "submit";
    form.appendChild(submitButton);

    popup.document.body.appendChild(title);
    popup.document.body.appendChild(form);

    form.addEventListener("submit", (event) => {
        event.preventDefault();
        popup.close();
        drawSquare(square.x, square.y, square.fillStyle);
    });
}

Neutralino.window.setTitle("Ma fenêtre NeutralinoJS");
Neutralino.window.setSize(300, 300);
Neutralino.events.on("load", () => {
    let newSquare = Object.assign({}, square);;
    squares.push(newSquare)
    drawSquare(newSquare.x, newSquare.y, square.fillStyle);
});
Neutralino.events.on('windowClose', () => {
    Neutralino.app.exit();
});
// getUsername();