// create fetch function and backend route to create posts -- remember to send credenetials --

import { createSignal } from "solid-js"

function Modal() {
  return (
    <div class="z-10 bg-blue-200 rounded-lg fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 ">
      <div class="grid grid-cols-2 grid-rows-6 min-w-fit min-h-fit">
        <div class="bg-white text-black row-span-1 col-span-1 flex justify-center">title1</div>
        <div class="bg-white text-black row-span-1 col-span-1 flex justify-center">title2</div>
        <div class="col-span-1 bg-purple-600">option1</div>
        <div class="col-span-1 bg-orange-600">setting1</div>
        <div class="col-span-1 bg-purple-600">option2</div>
        <div class="col-span-1 bg-orange-600">setting2</div>
        <div class="col-span-1 bg-purple-600">option3</div>
        <div class="col-span-1 bg-orange-600">setting3</div>
        <div class="col-span-1 bg-purple-600">option4</div>
        <div class="col-span-1 bg-orange-600">setting4</div>
        <div class="col-span-1 bg-purple-600">option5</div>
        <div class="col-span-1 bg-orange-600">setting5</div>



      </div>
    </div >
  )
}

export default function ProjectCreationModal() {
  const [modal, setModal] = createSignal(false)
  return (
    <>
      <button onClick={() => setModal(!modal())} class=" border rounded-lg border-blue-500 pr-2 pl-2  min-h-fit min-w-fit text-2xl lg:mr-3 font-bold text-blue-500 hover:text-yellow-500 hover:border-yellow-500 transition-colors duration-300">+</button>
      {modal() ? <Modal /> : (null)}
    </>
  )

}
