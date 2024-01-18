import { createSignal, onCleanup, type Setter } from "solid-js";

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

  async function handleLogin() {
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
      setEmail("");
      setPassword("");
      if (res.status == 401) {
        setMessage("UNAUTHORIZED");
        Notification();
      } else if (res.status == 500) {
        setMessage("INTERNL SERVER ERROR");
        Notification();
      }
    } catch (err) {
      console.log(`there was an error: ${err}`);
    }
  }

  return (
    <div class="min-h-screen flex items-center justify-center bg">
      {/* // */}
      {notif() && (
        <div class="fixed top-0 left-0 w-full bg-yellow-300 p-4 text-center">
          <p>{message()}</p>
        </div>
      )}
      {/* // */}
      <form
        onSubmit={handleLogin}
        class="bg-slate p-8 w-96  rounded-lg shadow-lg "
      >
        <div class="flex justify-center">
          <button
            onClick={() => props.setModal(false)}
            class="text-2xl font-bold mb-4 rounded-full p-2  pl-10 pr-10 bg-blue-400 text-white ease-in-out hover:bg-red-500 transition duration-300  transform hover:scale-105"
          >
            Login
          </button>
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
          <label for="password" class="block text-white text-sm font-bold mb-2">
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
          class="bg-blue-500 text-white p-1 rounded-md w-full ase-in-out hover:bg-red-500 transition duration-300  transform hover:scale-105 "
        >
          Submit
        </button>
      </form>
    </div>
  );
}
