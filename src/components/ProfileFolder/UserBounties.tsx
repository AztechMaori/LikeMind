import BountyItem from "./BountyItem";

let array = [1, 2, 3, 4, 5];
export default function UserBounties() {
  return (
    <div class="grid-cols-1">
      <div class="col-span-1  mr-2 ml-2 gap-2">
        {array.map((number, index) => (
          <BountyItem />
        ))}
      </div>
    </div>
  );
}
