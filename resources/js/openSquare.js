let popup = null;
let square = null;

let xInput = null;
let yInput = null;
let isDraggableInput = null;
let colorInput = null;
let form = null;

import { drawSquare } from "./vars.js";

const handleSubmit = (event) => {
    event.preventDefault();
    popup.close();
    square.fillStyle = colorInput.value;
    drawSquare(square.x, square.y, square.fillStyle);
};

const handleXInputChange = () => {
    square.x = Number(xInput.value);
};

const handleYInputChange = () => {
    square.y = Number(yInput.value);
};

const handleDraggableChange = () => {
    square.isDragable = isDraggableInput.checked;
};

const handleColorInputChange = () => {
    square.fillStyle = colorInput.value;
};

const updatePopupInputs = (squarep) => {
    square = squarep;

    xInput.value = square.x;
    yInput.value = square.y;
    isDraggableInput.checked = square.isDragable;
    colorInput.value = square.fillStyle;

    form.addEventListener("submit", handleSubmit);

    xInput.addEventListener("input", handleXInputChange);

    yInput.addEventListener("input", handleYInputChange);

    isDraggableInput.addEventListener("change", handleDraggableChange);

    colorInput.addEventListener("input", handleColorInputChange);
}

const openSquareContextMenu = (squarep) => {
    if (!popup || popup.closed) {
        popup = window.open("option.html", "popup", "width=300,height=400");
    } else {
        popup.focus();
        form.removeEventListener("submit", handleSubmit);
        xInput.removeEventListener("input", handleXInputChange);
        yInput.removeEventListener("input", handleYInputChange);
        isDraggableInput.removeEventListener("change", handleDraggableChange);
        colorInput.removeEventListener("input", handleColorInputChange);
        updatePopupInputs(squarep);
    }

    popup.onload = () => {
        xInput = popup.document.getElementById("x-input");
        yInput = popup.document.getElementById("y-input");
        isDraggableInput = popup.document.getElementById("is-draggable-input");
        colorInput = popup.document.getElementById("color-input");
        form = popup.document.getElementById("square-form");
        updatePopupInputs(squarep);
    }
};

export { openSquareContextMenu };