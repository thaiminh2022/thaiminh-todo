import { getDatabase, ref, child, get, set }
    from "https://www.gstatic.com/firebasejs/9.9.2/firebase-database.js";


export async function take_data(root_user) {
    let data = await (await getData(root_user));
    return data;
}

export function write_data(userId, data) {
    writeUserData(userId, data);
}

export async function getData(rootUser) {
    let final = "haha dit not await";

    const dbRef = ref(getDatabase());
    await get(child(dbRef, '' + rootUser)).then(async (snapshot) => {
        if (snapshot.exists()) {
            final = await snapshot.val();
        } else {
            final = "no data"
        }
    }).catch((error) => {
        final = "Error"
    });

    return final;
}

function writeUserData(userId, data) {
    const db = getDatabase();
    const json_object = JSON.parse(data);

    set(ref(db, '' + userId), json_object);
}