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

WebUI.navigateToUrl('https://prod-app.wageloch.com.au/')

WebUI.setText(findTestObject('Object Repository/Page_Wageloch Time and Attendance/input_Please Sign In_txtemailaddr'), 'jeriel_test@wageloch.com.au')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Wageloch Time and Attendance/input_Please Sign In_txtpassword'), 
    'p4y+y39Ir5PJb2ispxT0Ew==')

WebUI.click(findTestObject('Object Repository/Page_Wageloch Time and Attendance/input_Remember Me_btnlogin'))

WebUI.click(findTestObject('Object Repository/Page_Wageloch - Payroll/button_No'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Wageloch - Payroll/select_Central PayrollCentral Site 1Central_220952'), 
    '_tst_138051_5', true)

WebUI.click(findTestObject('Object Repository/Page_Wageloch - Rosters/a_Staff'))

WebUI.click(findTestObject('Object Repository/Page_Wageloch - Staff/span_Create'))

WebUI.setText(findTestObject('Object Repository/Page_Wageloch - Staff/input_First Name_ctl00MainContentucStaffEdi_403476'), 
    'Stafffn01')

WebUI.setText(findTestObject('Object Repository/Page_Wageloch - Staff/input_Surname_ctl00MainContentucStaffEdit2t_8ee10f'), 
    'Staffsn01')

WebUI.setText(findTestObject('Object Repository/Page_Wageloch - Staff/input_First Name_ctl00MainContentucStaffEdi_403476'), 
    'Stafffn01')

WebUI.click(findTestObject('Object Repository/Page_Wageloch - Staff/input_Payroll Code_ctl00MainContentucStaffE_582538'))

WebUI.setText(findTestObject('Object Repository/Page_Wageloch - Staff/input_Email_ctl00MainContentucStaffEdit2txtEmail'), 
    'Stafffn01@gmail.com')

WebUI.click(findTestObject('Object Repository/Page_Wageloch - Staff/label_Send Onboarding Link to Employee'))

WebUI.click(findTestObject('Object Repository/Page_Wageloch - Staff/a_DONE'))

WebUI.closeBrowser()

