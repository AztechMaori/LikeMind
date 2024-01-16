export default function Bio() {
  const username = "Ash";
  const location = "Brisbane, Australia";
  const rating = 9.87;
  return (
    <>
      <div class="flex flex-col md:flex-row">
        <div class="bg-white p-8 rounded-lg shadow-md max-w-md w-full md:w-1/2 lg:w-1/3 flex items-center">
          <img
            src="https://placekitten.com/150/150"
            alt="Profile Picture"
            class="w-20 h-20 rounded-full mr-4 md:w-32 md:h-32 lg:w-40 lg:h-40 translate-y-[-50%]"
          />
          <div class="md:ml-4">
            <h2 class="text-xl font-semibold text-gray-800">John Doe</h2>
            <p class="text-gray-500">Web Developer</p>
            <div class="mt-6">
              <p class="text-gray-700">{username}</p>
              <p class="text-gray-700">{location}</p>
              <p class="text-gray-700">
                Rating:
                <span class="rounded-full bg-purple-400 text-white px-2 py-0 ml-2 transition duration-300 ease-in-out transform hover:bg-green-500 hover:scale-105">
                  {rating}
                </span>
              </p>
            </div>

            <div class="mt-6 flex">
              <a
                href="#"
                class="ml-0  px-4 py-2 rounded-full bg-blue-500 text-white transition duration-300 ease-in-out transform hover:bg-blue-600 hover:scale-105"
              >
                Follow
              </a>
              <a
                href="#"
                class="ml-4 px-4 py-2 rounded-full bg-red-500 text-white transition duration-300 ease-in-out transform hover:bg-red-600 hover:scale-105"
              >
                Message
              </a>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
