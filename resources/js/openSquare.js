let popup = null;

import { drawSquare } from "./vars.js";

const updatePopupInputs = (square) => {
    const xInput = popup.document.getElementById("x-input");
    const yInput = popup.document.getElementById("y-input");
    const isDraggableInput = popup.document.getElementById("is-draggable-input");
    const colorInput = popup.document.getElementById("color-input");
    const form = popup.document.getElementById("square-form");

    xInput.value = square.x;
    yInput.value = square.y;
    isDraggableInput.checked = square.isDragable;
    colorInput.value = square.fillStyle;

    form.addEventListener("submit", (event) => {
        event.preventDefault();
        popup.close();
        square.fillStyle = colorInput.value;
        drawSquare(square.x, square.y, square.fillStyle);
    });

    xInput.addEventListener("input", () => {
        square.x = Number(xInput.value);
    });

    yInput.addEventListener("input", () => {
        square.y = Number(yInput.value);
    });

    isDraggableInput.addEventListener("change", () => {
        square.isDragable = isDraggableInput.checked;
    });
}

const openSquareContextMenu = (square) => {
    if (!popup || popup.closed) {
        popup = window.open("option.html", "popup", "width=300,height=400");
    } else {
        popup.focus();
        updatePopupInputs(square);
    }

    popup.onload = () => {
        updatePopupInputs(square);
    }
};

export { openSquareContextMenu };