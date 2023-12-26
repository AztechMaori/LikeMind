import { createSignal } from "solid-js";

export default function SignUp() {
  const [username, setUsername] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  async function handleSubmit() {
    const user_data = {
      username: username(),
      email: email(),
      password: password(),
    };

    const url = "http://localhost:3000/route";

    try {
      const response = await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(user_data),
      });
    } catch (err) {
      console.log(err);
    }
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
    </div>
  );
}
