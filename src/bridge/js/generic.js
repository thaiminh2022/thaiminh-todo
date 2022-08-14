export async function hello(msg) {
    console.log(msg);
}

export async function fetch_data() {
    let data = (await fetch("https://www.random.org/integers/?num=1&min=1&max=100&col=1&base=10&format=plain&rnd=new")).text();

    return data;
}
export function log(msg) {
    console.log(msg);
}
export function log_json(json) {
    const json_data = JSON.parse(json);
    console.log(json_data);
}