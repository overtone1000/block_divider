export let handle_error = (e: Error) => {
    alert("Error (see developer console)");
    console.error(e);
};

export enum DisplayMode {
    Loading,
    List,
    Create,
    Modify
}