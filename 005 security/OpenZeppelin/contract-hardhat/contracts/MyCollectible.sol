// import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

// -contract MyCollectible is ERC721 {
contract MyCollectible is ERC721Upgradeable {
 //    -    constructor() ERC721("MyCollectible", "MCO") public {
    function initialize() initializer public {
        __ERC721_init("MyCollectible", "MCO");
     }
}