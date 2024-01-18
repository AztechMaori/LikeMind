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

  async function handleSignIn() {
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
      setUsername("");
      setPassword("");
      setEmail("");
      console.log(response.status);

      if (response.status == 401) {
        setMessage("UNAUTHORIZED");
        Notification();
      } else if (response.status == 500) {
        setMessage("INTERNAL SERVER ERROR");
        Notification();
      }
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

  <button class=" p-0.5 pl-1 pr-1 rounded-full bg-slate-600 text-white transition duration-300 ease-in-out hover:bg-purple-700">
    Go To Login
  </button>;

  return (
    <div class="min-h-screen flex items-center justify-center overflow-auto ">
      {notif() && (
        <div class="fixed top-0 left-0 w-full bg-yellow-300 p-4 text-center">
          <p>{message()}</p>
        </div>
      )}
      <form
        onSubmit={handleSignIn}
        class="bg-black p-8 w-96 rounded-lg shadow-lg"
      >
        <div class="flex justify-center">
          <button
            onClick={() => props.setModal(true)}
            class="text-2xl font-bold mb-4 rounded-full p-2 pr-10 pl-10 bg-blue-400 text-white ease-in-out hover:bg-red-500 transition duration-300  transform hover:scale-105"
          >
            Sign Up
          </button>
        </div>
        <div class="mb-4">
          <label for="username" class="block text-white text-sm font-bold mb-2">
            Username
          </label>
          <input
            type="text"
            id="username"
            class="w-full p-2  border border-gray-300 rounded-md"
            value={username()}
            onInput={(e) => setUsername(e.target.value)}
            required
          />
        </div>
        <div class="mb-4">
          <label for="email" class="block text-white text-sm font-bold mb-2">
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
            class="block text-white  text-sm font-bold mb-2"
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
          class="bg-blue-500 text-white p-2 rounded-md w-full ase-in-out hover:bg-red-500 transition duration-300  transform hover:scale-105"
        >
          Submit
        </button>
      </form>
    </div>
  );
}
