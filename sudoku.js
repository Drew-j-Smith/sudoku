function createNode(arr) {
    const table = document.createElement("table");
    for (let row = 0; row < 9; row++) {
        let row_node = document.createElement("tr");
        for (let col = 0; col < 9; col++) {
            let entry = document.createElement("td");
            let data = arr[row * 9 + col] >= 1 && arr[row * 9 + col] <= 9 ? arr[row * 9 + col] : "";
            let text = document.createTextNode(data);
            entry.appendChild(text);
            row_node.appendChild(entry);
        }
        table.appendChild(row_node);
    }
    return table;
}

import init, { complete_sudoku } from './pkg/sudoku.js';
(async () => {
    await init();
    let response = await fetch('https://sugoku.herokuapp.com/board?difficulty=easy');
    let raw_json = await response.json();
    let data = raw_json.board.reduce((arr, curr) => arr.concat(curr));
    document.getElementById("unsolved").insertAdjacentElement('afterend', createNode(data));

    let res = complete_sudoku(data);
    console.log(res);

    document.getElementById("solved").insertAdjacentElement('afterend', createNode(res));
})();