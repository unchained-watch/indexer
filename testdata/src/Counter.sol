// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    event SetNumber(uint newNumber);

    event SetNumberBy(uint newNumber, address sender);

    event Increment(uint previousNumber, uint newNumber);
    event Decrement(uint previousNumber, uint newNumber);

    error NumberTooLow(uint targetNumber);

    function setNumber(uint256 newNumber) public {
        number = newNumber;
        emit SetNumber(newNumber);
        emit SetNumberBy(newNumber, msg.sender);
    }

    function increment() public {
        uint previousNumber = number;
        number++;
        emit Increment(previousNumber, number);
    }

    function decrement() public {
        if (number <= 42) {
            revert NumberTooLow({targetNumber: number - 1});
        }
        uint previousNumber = number;
        number--;
        emit Increment(previousNumber, number);
    }
}
