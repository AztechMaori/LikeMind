import { createSignal, onCleanup, type Setter } from "solid-js";
interface Props {
  setModal: Setter<boolean>;
}

export default function SignUp(props: Props) {
  const [username, setUsername] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  const [notif, setNotif] = createSignal(false);
  const [message, setMessage] = createSignal("");

  function Notification() {
    setNotif(true);

    const timeout = setTimeout(() => {
      setNotif(false);
    }, 1300);

    onCleanup(() => {
      clearTimeout(timeout);
    });
  }

  async function handleSubmit() {
    event?.preventDefault();

    const user_data = {
      username: username(),
      email: email(),
      password: password(),
    };

    const url = "http://localhost:3000/signup";

    try {
      const response = await fetch(url, {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(user_data),
      });
      console.log(response.status);

      if (response.status == 401) {
        setMessage("UNAUTHORIZED");
        Notification();
      } else if (response.status == 500) {
        setMessage("INTERNAL SERVER ERROR");
        Notification();
      }

      setUsername("");
      setPassword("");
      setEmail("");
    } catch (err) {
      console.log(err);
    }
  }

  async function cookie_backend() {
    const url = "http://localhost:3000/time";

    const res = fetch(url, {
      method: "GET",
      credentials: "include",
    });
  }

  async function trial() {
    const url = "http://localhost:3000/check";
    try {
      const res = await fetch(url, {
        method: "GET",
        credentials: "include",
      });
      console.log(`succesfull operation ${res.status}`);
    } catch (err) {
      console.log(`there was an error, ${err}`);
    }
  }

  return (
    <div class="min-h-screen flex items-center justify-center ">
      {notif() && (
        <div class="fixed top-0 left-0 w-full bg-yellow-300 p-4 text-center">
          <p>Unauthorized</p>
        </div>
      )}
      <form
        onSubmit={handleSubmit}
        class="bg-white p-8 shadow-md rounded-md w-96"
      >
        <button onClick={() => props.setModal(true)} class=" mb-4">
          click here to go to Login
        </button>
        <h2 class="text-2xl font-bold mb-4">Sign Up</h2>
        <div class="mb-4">
          <label
            for="username"
            class="block text-gray-700 text-sm font-bold mb-2"
          >
            Username
          </label>
          <input
            type="text"
            id="username"
            class="w-full p-2 border border-gray-300 rounded-md"
            value={username()}
            onInput={(e) => setUsername(e.target.value)}
            required
          />
        </div>
        <div class="mb-4">
          <label for="email" class="block text-gray-700 text-sm font-bold mb-2">
            Email
          </label>
          <input
            type="email"
            id="email"
            class="w-full p-2 border border-gray-300 rounded-md"
            value={email()}
            onInput={(e) => setEmail(e.target.value)}
            required
          />
        </div>
        <div class="mb-4">
          <label
            for="password"
            class="block text-gray-700 text-sm font-bold mb-2"
          >
            Password
          </label>
          <input
            type="password"
            id="password"
            class="w-full p-2 border border-gray-300 rounded-md"
            value={password()}
            onInput={(e) => setPassword(e.target.value)}
            required
          />
        </div>
        <button
          type="submit"
          class="bg-blue-500 text-white p-2 rounded-md w-full"
        >
          Sign Up
        </button>
      </form>
      <button onClick={cookie_backend}>check if cookie exists</button>
      <button onClick={trial}>check if middleware works</button>
    </div>
  );
}
