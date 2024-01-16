import { createSignal, onCleanup, type Setter } from "solid-js";

// Login States.
const [email, setEmail] = createSignal("");
const [password, setPassword] = createSignal("");
const [notif, setNotif] = createSignal(false);
const [message, setMessage] = createSignal("");

// Sign-Up states
const [username, setUsername] = createSignal("");
