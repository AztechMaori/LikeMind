


export default function NewIdea() {
  return (
    <>
      <div class=" grid grid-cols-5 min-w-fit  bg-black rounded-lg pr-4 pl-2 pb-2 ">
        <div class="col-span-5 flex justify-center pt-0.5 pb-1 mb-2 border-b-white border-black border text-white">TITLE</div>
        <div class=" flex col-span-2 flex-col justify-between gap-2 border border-black border-r-white pr-2 ">
          <button class="bg-red-400 rounded-full pr-4 pl-4 flex hover:text-white duration-300">
            3/4
          </button>

          <button class="bg-blue-400 rounded-full pr-4 pl-4 transform hover:scale-105 hover:text-white duration-300">
            info
          </button>

          <button class="bg-orange-500 rounded-full pr-4 pl-4 hover:text-white hover:scale-105 duration-300">
            pin
          </button>


          <button class="bg-green-500 rounded-full flex justify-center pl-4 pr-4 hover:text-white transform hover:scale-105 duration-300">
            join
          </button></div>
        <div class="flex col-span-3 ml-2 text-white">image</div>

      </div>

    </>
  )
}
