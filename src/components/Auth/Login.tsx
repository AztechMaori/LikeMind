import { createSignal, onCleanup, type Setter } from "solid-js";
import Notification from "../Notifications/authNotifs";

interface Props {
  setModal: Setter<boolean>;
}

export default function Login(props: Props) {
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
      email: email(),
      password: password(),
    };
    const url = "http://localhost:3000/login";
    try {
      const res = await fetch(url, {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(user_data),
      });
      console.log(`the response was: ${res.status}`);
      if (res.status == 401) {
        setMessage("UNAUTHORIZED");
        Notification();
      } else if (res.status == 500) {
        setMessage("INTERNL SERVER ERROR");
        Notification();
      }
      setEmail("");
      setPassword("");
    } catch (err) {
      console.log(`there was an error: ${err}`);
    }
  }

  return (
    <div class="min-h-screen flex items items-center justify-center ">
      {notif() && (
        <div class="fixed top-0 left-0 w-full bg-yellow-300 p-4 text-center">
          <p>{message()}</p>
        </div>
      )}
      <form
        onSubmit={handleSubmit}
        class="bg-white p-8 shadow-md rounded-md w-96"
      >
        <button onclick={() => props.setModal(false)} class=" mb-4">
          go to SignIn
        </button>
        <h2 class="text-2xl font-bold mb-4">Login</h2>
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
          Login
        </button>
      </form>
    </div>
  );
}
