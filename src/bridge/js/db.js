import { getDatabase, ref, child, get, set }
    from "https://www.gstatic.com/firebasejs/9.9.2/firebase-database.js";



export async function take_data() {
    let data = await (await getData());
    console.log("Object: ", data);
    return data;
}

export function write_data(userId, data) {
    writeUserData(userId, data);
}

export async function getData() {
    let final = "haha dit not await";

    const dbRef = ref(getDatabase());
    await get(child(dbRef, `user_1`)).then(async (snapshot) => {
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