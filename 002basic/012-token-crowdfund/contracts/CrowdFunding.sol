// SPDX-License-Identifier: MIT 
pragma solidity 0.8.23;
// Importing OpenZeppelin's SafeMath Implementation
import './SafeMath.sol';
import './Project.sol';

contract CrowdFunding {
    using SafeMath for uint256;
    // List of existing project
    Project[] projects;

    // Event that will be emitted whenever a new project is started.
    // 每当启动新项目时将发出的事件。
    event ProjectStarted (
        address contractAddress,
        address projectStarter,
        string projectTitle,
        string projectDesc,
        uint256 deadline,
        uint256 goalAmout
    );

    /** @dev Function to get all projects' contract addresses.获取所有项目的合约地址。
      * @return A list of all projects' contract addresses 返回所有工程合约地址
      */
    function returnAllProjects() external view returns(Project[] memory){
        return projects;
    }
    /** @dev Function to start a new project.  启动一个新项目
      * @param title Title of the project to be created 新建项目的名称
      * @param description Brief description about the project 新建项目的描述
      * @param durationInDays Project deadline in days 项目的截止日期
      * @param amountToRaise Project goal in wei  项目募集的目标金额wei
      */
    function startProject(
        string calldata title,
        string calldata description,
        uint durationInDays,
        uint amountToRaise
    ) external {
        uint raiseUntil = block.timestamp.add(durationInDays.mul(1 days));
        address payable  starter = payable(msg.sender) ;
        Project newProject = new Project(starter,title,description,raiseUntil,amountToRaise);
        projects.push(newProject);
        emit ProjectStarted(
            address(newProject),
            msg.sender,
            title,
            description,
            raiseUntil,
            amountToRaise
        );
    }
}

