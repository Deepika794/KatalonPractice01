import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.practo.com/doctors')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4'))

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1'), 
    'Bangalor')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2'), 
    'Bangalo')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3'), 
    'Bangal')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4'), 
    'Banga')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5'), 
    'Bang')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6'), 
    'Ban')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7'), 
    'Ba')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8'), 
    'B')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9'), 
    '')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10'), 
    'P')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11'), 
    'Pu')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12'), 
    'Pun')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13'), 
    'Pune')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/div_Pune'))

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14'), 
    'J')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15'), 
    'Ju')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16'), 
    'Jup')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17'), 
    'Jupi')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18'), 
    'Jupit')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19'), 
    'Jupitr')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18'), 
    'Jupit')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20'), 
    'Jupite')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/input_Find and Book_c-omni-searchbox c-omni_58efd4_1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18_19_20_21'), 
    'Jupiter')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo  Book Doctor Appointments Onlin_0c7a74/div_Baner'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Jupiter Hospital, Multi Speciality Hos_eeb131/span_specialities'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Jupiter Hospital, Multi Speciality Hos_eeb131/span_more'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Jupiter Hospital, Multi Speciality Hos_eeb131/div_laparoscopic surgeon (5)'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Jupiter Hospital, Multi Speciality Hos_eeb131/h2_Dr. Satish Pattanshetti'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/span_Doctor friendliness'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/span_Stories'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/span_Consult QA'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/span_Healthfeed'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/input_Login  Signup_c-omni-searchbox c-omni_a9b18a'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/div_Pimpri-Chinchwadpune'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Satish Pattanshetti - Bariatric Su_e3a4d0/div_Dentist'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Find Best Dentists in Pimpri-chinchwad_e1b98c/h2_Dr. Sandeep Bhirud'))

WebUI.switchToWindowTitle('Dr. Sandeep Bhirud - Cosmetic/Aesthetic Dentist - Book Appointment Online, View Fees, Feedbacks | Practo')

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Sandeep Bhirud - CosmeticAesthetic_8b9d9f/li_Stories(917)'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Sandeep Bhirud - CosmeticAesthetic_8b9d9f/span_Healthfeed'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Sandeep Bhirud - CosmeticAesthetic_8b9d9f/span_For Providers'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Dr. Sandeep Bhirud - CosmeticAesthetic_8b9d9f/span_Software for providers'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Software solutions for Doctors, Clinic_74bc80/a_Learn more'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Security - Practo/h3_Your data has multiple encrypted backups'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Security - Practo/div_Careers'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Security - Practo/div_Careers_1'))

WebUI.closeBrowser()

