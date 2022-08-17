// Import the functions you need from the SDKs you need
import { initializeApp } from "https://www.gstatic.com/firebasejs/9.9.2/firebase-app.js";

// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
const firebaseConfig = {

    apiKey: "AIzaSyA3GGKrwiQocDKNyVZCec-Q1_-CbFqQA6Y",

    authDomain: "todro-d5ef1.firebaseapp.com",

    databaseURL: "https://todro-d5ef1-default-rtdb.firebaseio.com",

    projectId: "todro-d5ef1",

    storageBucket: "todro-d5ef1.appspot.com",

    messagingSenderId: "559860599189",

    appId: "1:559860599189:web:0f978639f445317092ce9c"

};


// Initialize Firebase
// Everything needed for database usage
export const app = initializeApp(firebaseConfig);

