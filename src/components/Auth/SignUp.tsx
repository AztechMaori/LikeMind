import { createSignal } from "solid-js";

export default function SignUp() {
  const [username, setUsername] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  async function handleSubmit() {
    event?.preventDefault();

    const user_data = {
      username: username(),
      email: email(),
      password: password(),
    };

    const url = "http://localhost:3000/route";

    try {
      const response = await fetch(url, {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(user_data),
      });
      console.log(response);
      setUsername("");
      setPassword("");
      setEmail("");
    } catch (err) {
      console.log(err);
    }
  }

  function cookie() {
    const url = "http://localhost:3000/set";
    try {
      let res = fetch(url, {
        credentials: "include",
      });
      console.log(res);
    } catch (err) {
      console.log(err);
    }
  }

  function cookie_backend() {
    const url = "http://localhost:3000/get";
    try {
      let res = fetch(url, {
        credentials: "include",
      });
      console.log(`it worked! ${res}`);
    } catch (err) {
      console.log(`the error is ${err}`);
    }
  }

  function show_doc_cookie() {
    console.log(document.cookie);
  }

  return (
    <div class="min-h-screen flex items-center justify-center">
      <form
        onSubmit={handleSubmit}
        class="bg-white p-8 shadow-md rounded-md w-96"
      >
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
      <button onClick={cookie}>setting cookie</button>
      <button onClick={cookie_backend}>cookie backend</button>
      <button onClick={show_doc_cookie}> show broswer boockies</button>
    </div>
  );
}
