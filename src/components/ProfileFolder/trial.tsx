export default function Profile() {
  const username = "Ash";
  const location = "Brisbane, Australia";
  const rating = 9.87;
  return (
    <div class="grid grid-cols-4 grid-rows-3 bg-white pl-4 pt-2 pr-2 pb-0 rounded-lg shadow-md  w-full  ">
      <div class="flex justify-center">
        <div class=" col-span-1 row-span-2">
          <img
            src="https://placekitten.com/150/150"
            alt="Profile Picture"
            class="w-20 h-20 ml-2 rounded-full "
          />
        </div>
      </div>
      <div class="col-span-2 row-span-2 ml-2  pl-7 lg:ml-2 xl:ml-0">
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
        <div class="col-span-2 row-span-1">
          <div class=" absolute ">
            <div class=" relative top-6 right-8 ">
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
              <div class="col-span-1"> socialmedia</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
