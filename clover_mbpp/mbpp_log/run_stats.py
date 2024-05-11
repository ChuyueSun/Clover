total_verified_ids =  ['105', '106', '113', '126', '133', '135', '139', '14', '142', '143', '145', '161', '17', '170', '171', '18', '2', '230', '233', '234', '238', '240', '242', '249', '251', '257', '261', '262', '264', '266', '268', '269', '273', '276', '279', '282', '284', '290', '292', '3', '304', '307', '309', '310', '312', '396', '397', '399', '401', '406', '412', '414', '424', '426', '430', '432', '436', '445', '452', '455', '457', '458', '460', '461', '470', '476', '554', '555', '557', '565', '57', '572', '574', '577', '578', '581', '586', '587', '59', '591', '598', '603', '606', '61', '618', '622', '623', '626', '637', '641', '69', '70', '728', '747', '750', '751', '759', '762', '769', '776', '79', '790', '792', '799', '8', '80', '801', '804', '808', '809', '82', '85', '86', '89', '94', '95']
strong_post_ids =   ['105', '106', '113', '133', '135', '139', '14', '142', '143', '145', '161', '17', '170', '171', '18', '2', '230', '233', '234', '238', '240', '242', '249', '251', '257', '261', '262', '264', '266', '268', '269', '273', '276', '279', '282', '284', '290', '292', '3', '307', '309', '310', '312', '396', '397', '399', '401', '406', '412', '414', '424', '426', '436', '445', '452', '455', '457', '458', '460', '461', '476', '554', '555', '557', '565', '574', '577', '578', '581', '587', '59', '591', '598', '606', '618', '623', '626', '637', '641', '69', '70', '728', '750', '751', '762', '769', '776', '79', '790', '792', '799', '8', '80', '801', '804', '808', '809', '82', '85', '86', '89', '94', '95']
import json

def read_json_file(file_path):

    with open(file_path, 'r', encoding='utf-8') as file:
        data = json.load(file)
    data = data["gt"]
    accept = 0
    accept_list = []
    for task_id in data:
        if data[task_id][0]==True:
            accept+=1
            accept_list.append(task_id.split("_")[-1])

    metrics = calculate_metrics(set(strong_post_ids), set(accept_list), set(total_verified_ids))
    file_path = "RQ3_stats.json"

    with open(file_path, 'w', encoding='utf-8') as file:
        json.dump(metrics, file, ensure_ascii=False, indent=4)

    print(metrics)
    return metrics


def calculate_metrics(actual_set, predicted_set, universal_set):
    """
    Calculate the true positives, true negatives, false positives, and false negatives.

    Args:
        actual_set (set): The set of actual labels.
        predicted_set (set): The set of predicted labels.
        universal_set (set, optional): The universal set of all possible labels. Required for TN.

    Returns:
        dict: A dictionary containing the counts for TP, TN, FP, FN.
    """
    print("universal set len: ", len(total_verified_ids))
    print("actual set len: ", len(actual_set))
    print("predicted set len: ", len(predicted_set))
    tp = actual_set & predicted_set
    fp = predicted_set - actual_set
    fn = actual_set - predicted_set
    tn = universal_set - actual_set - predicted_set
    
    return {
        "TP": list(tp),
        "TN": list(tn),
        "FP": list(fp),
        "FN": list(fn)
    }

# Example usage:
file_path = 'log_results_k_10.log'
read_json_file(file_path)
