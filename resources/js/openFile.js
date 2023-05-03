const filters = [
    { name: "JSON files", extensions: ["json"] },
]

const openFile = async () => {
    const [filename] = await Neutralino.os.showOpenDialog(
        'Open a spreadsheet',
        { filters: filters, multiSelections: false }
    );
    return filename;
}

export { openFile };