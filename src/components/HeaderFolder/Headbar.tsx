export default function Headbar() {
  const User_Auth = "Login";
  return (
    <>
      <header class="bg-black text-white p-4">
        <div class="container mx-auto">
          <div class="flex justify-between items-center">
            <a
              href="#"
              class="text-2xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
            >
              LIKEMIND
            </a>
            <nav class="space-x-4">
              <button class=" border rounded-full  p-1 pr-3 pl-3 min-h-fit min-w-fit text-2xl lg:mr-3 font-bold text-blue-500 hover:text-yellow-500 hover:border-yellow-500 transition-colors duration-300">+</button>
              <a
                href="/auth/"
                class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
              >
                {User_Auth}
              </a>
              <a
                href="/"
                class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
              >
                Home
              </a>
              <a
                href="/trendingpage/"
                class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
              >
                Profile
              </a>

              <a
                href="/profilepage/"
                class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
              >
                Statistics
              </a>
            </nav>
          </div>
        </div>
      </header>
    </>
  );
}
