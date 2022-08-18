export function set_item_storage(key,item){
    localStorage.setItem(key, item);
}
export function get_item_storage(key) {
    let data=  localStorage.getItem(key);

    if(data=== null)
    {
       throw new Error("No data found");
    }

    return data;
}

export function clear_item_storage(){
    localStorage.clear();
}