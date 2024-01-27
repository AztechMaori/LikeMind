
interface ProjectItemProps {
  ProjectName: String;
  PeopleNumber: Number;
}

export default function ProjectItem() {
  const ProjectName = "cool job idea, check info for more!";
  const people = 4;

  return (
    <>
      <div class="grid grid-col-2 rounded-lg shadow-lg p-4 bg-black pr-5 pl-5  ">
        <h2 class="col-span-2 border-white border-b-2 text-lg font-bold   text-white flex justify-center mb-4 pb-1 pl-3   hover:border-purple-500 duration-300  ">
          {ProjectName}
        </h2>


        <div class="col-span-2 rounded-full  flex flex-row justify-center gap-2 ">
          <button class="bg-red-400 rounded-full pr-4 pl-4 flex hover:text-white duration-300">
            {people}
          </button>
          <button class="bg-blue-400 rounded-full pr-4 pl-4 transform hover:scale-105 hover:text-white duration-300">
            info
          </button>

          <button class="bg-orange-500 rounded-full pr-4 pl-4 hover:text-white hover:scale-105 duration-300">
            pin
          </button>


          <button class=" bg-green-500 rounded-full flex justify-center pl-4 pr-4 hover:text-white transform hover:scale-105 duration-300">
            join
          </button>
        </div>
      </div>
    </>
  )
}
