import ProjectItem from "./newidea";

interface ProjectScrollBarProps {
  title: String;
}

export default function ProjectScrollBar(props: ProjectScrollBarProps) {
  let array = [1, 2, 3, 4, 5, 6];

  let Title: String = props.title;
  return (
    <>
      <h3 class="flex justify-center bg-black  text-lg border-2 border-black  ">
        <div class="flex justify-center pl-4 pr-4 text-white mt-2 mb-2 border rounded-full shadow-lg hover:border-yellow-500  transform hover:scale-105  duration-300  ">
          {Title}
        </div>
      </h3>
      <div class="flex flex-col gap-4 bg-blue-600 overflow-x-auto custom-scrollbar">
        <div>
          <div class=" flex flex-row gap-4 ml-2 mt-6 mb-6">
            {array.map((number, index) => (
              <ProjectItem />
            ))}
          </div >
        </div>
      </div>
    </>
  );
}
