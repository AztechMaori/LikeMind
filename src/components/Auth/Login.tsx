import { createSignal, type Setter } from "solid-js";

interface Props {
  setModal: Setter<boolean>;
}

export default function Login(props: Props) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  function handleSubmit() {}

  return (
    <div class="min-h-screen flex items items-center justify-center ">
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
