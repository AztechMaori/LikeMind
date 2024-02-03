import Headbar from "../HeaderFolder/Headbar";
import ProjectScrollBar from "./Project.tsx"
import Footer from "./Footer.tsx"

// ADD FUNCTION TO FETCH POSTS AND PASS THE STATUS CODE AS A PROP TO THE HEADER function to set the login or logout button

export default function HomepageWrapper() {
  return (
    <div class=" grid-rows-5 bg-black">
      <div class="row-span-1 bg-teal-400">
        <Headbar authenticated={true} />
      </div>
      <div class="row-span-3 bg-blue-400">
        <ProjectScrollBar title={"Highest-Rated"} />
        <ProjectScrollBar title={"For You"} />
        <ProjectScrollBar title={"Yours"} />
        <Footer />
      </div>
    </div>
  )
}
