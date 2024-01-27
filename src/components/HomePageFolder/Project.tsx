import ProjectItem from "./ProjectItem";
import NewIdea from "./newidea";

interface ProjectScrollBarProps {
  posts: [];
  title: String;
}

export default function ProjectScrollBar() {
  let array = [1, 2, 3, 4, 5, 6];
  let Title = "Popular";
  return (
    <>
      <h3 class="flex justify-center bg-black  text-lg border-2 border-black  ">
        <div class="flex justify-center pl-4 pr-4 text-white mt-2 mb-2 border rounded-full shadow-lg hover:border-orange-500 duration-300  ">
          {Title}
        </div>
      </h3>
      <div class="flex flex-col gap-4 bg-pink-400 overflow-x-auto custom-scrollbar">
        <div>
          <div class=" flex flex-row gap-4 ml-2 mt-6 mb-6">
            {array.map((number, index) => (
              //<ProjectItem />
              <NewIdea />

            ))}
          </div>
        </div>
      </div>
    </>
  );
}
