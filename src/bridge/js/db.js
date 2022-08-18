import { getDatabase, ref, child, get, set }
    from "https://www.gstatic.com/firebasejs/9.9.2/firebase-database.js";

import {get_user_uid} from "./auth.js";


export async function take_data(root_user, path) {
    let data = await (await getData(root_user, path));

    

    return data;
}

export function write_data(userId, path, data) {
    writeUserData(userId, path, data);
    
}

export async function getData(rootUser, path) {
    let final = "haha dit not await";

    const dbRef = ref(getDatabase());
    await get(child(dbRef, '' + rootUser + path)).then(async (snapshot) => {
        if (snapshot.exists()) {
            final = await snapshot.val();
        } else {
            final = "no data"
        }
    }).catch((_) => {
        final = "Error"
    });

    return final;
}

function writeUserData(userId, path,data) {
    const db = getDatabase();
    const json_object = JSON.parse(data);

    set(ref(db, '' + userId + path), json_object);
}