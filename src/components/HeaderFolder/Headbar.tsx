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
                Trending
              </a>
              <a
                href="/jobspage/"
                class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
              >
                Jobs
              </a>
              <a
                href="/profilepage/"
                class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300"
              >
                Profile
              </a>
            </nav>
          </div>
        </div>
      </header>
    </>
  );
}
