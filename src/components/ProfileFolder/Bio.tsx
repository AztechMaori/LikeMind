import SocialMedia from "./SocialMedia";

export default function Bio() {
  const username = "Ash";
  const status = "Online";
  const location = "Brisbane Australia";
  const rating = 9.87;
  return (
    <>
      <div class="grid grid-cols-3">
        <div class="col-span-1">
          <div class="flex justify-center pr-3">
            <img
              src="https://placekitten.com/150/150"
              alt="Profile Picture"
              class="w-25 h-25 ml-2 rounded-full "
            />
          </div>
        </div>
        <div class="col-span-1">
          <div class="flex flex-col">
            <div class="">
              <h2 class="text-xl font-semibold text-gray-800">{username}</h2>
              <p class="text-gray-500">Web Developer</p>
              <div class="mt-6">
                <p class="text-gray-700 pb-3">
                  Status: <span>{status}</span>
                </p>
                <p class="text-gray-700 pb-3 ">Location: {location}</p>
                <p class="text-gray-700">
                  Rating:
                  <span class="rounded-full bg-purple-400 text-white px-2 py-0 ml-2 transition duration-300 ease-in-out transform hover:bg-green-500 hover:scale-105">
                    {rating}
                  </span>
                </p>
              </div>
              <div class=" pt-2 mt-2 mb-4">
                <button class="ml-0 mb-1 mr-2 px-4 py-2 rounded-full bg-blue-500 text-white transition duration-300 ease-in-out transform hover:bg-blue-600 hover:scale-105">
                  Follow
                </button>
                <button class="ml-0 px-4 py-2 rounded-full bg-red-500 text-white transition duration-300 ease-in-out transform hover:bg-red-600 hover:scale-105">
                  Message
                </button>
              </div>
            </div>
          </div>
        </div>
        <div class="col-span-1 flex justify-center ">
          <SocialMedia />
        </div>
      </div>
    </>
  );
}
