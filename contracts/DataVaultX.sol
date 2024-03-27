// DataVaultX.sol - Smart contract for data transactions

pragma solidity ^0.8.0;

contract DataVaultX {
    // Define data transaction structure
    struct DataTransaction {
        address seller;
        string data;
        uint256 price;
        bool isSold;
    }

    // Mapping from data transaction ID to DataTransaction struct
    mapping(uint256 => DataTransaction) public dataTransactions;

    // Event for data transaction creation
    event DataTransactionCreated(uint256 transactionId, address seller, uint256 price);

    // Function to create a new data transaction
    function createDataTransaction(string memory _data, uint256 _price) external {
        // Increment transaction ID
        uint256 transactionId = dataTransactions.length;

        // Create new data transaction
        dataTransactions[transactionId] = DataTransaction(msg.sender, _data, _price, false);

        // Emit event
        emit DataTransactionCreated(transactionId, msg.sender, _price);
    }

    // Function to buy data transaction
    function buyDataTransaction(uint256 _transactionId) external payable {
        // Require transaction exists and is not sold
        require(_transactionId < dataTransactions.length, "Transaction does not exist");
        require(!dataTransactions[_transactionId].isSold, "Transaction already sold");
        require(msg.value >= dataTransactions[_transactionId].price, "Insufficient funds");

        // Transfer payment to seller
        payable(dataTransactions[_transactionId].seller).transfer(msg.value);

        // Mark transaction as sold
        dataTransactions[_transactionId].isSold = true;
    }
}
